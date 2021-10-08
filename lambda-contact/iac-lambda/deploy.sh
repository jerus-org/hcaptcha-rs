#!/bin/sh +eux
clear
# Parameters
DEP_VERSION="v3"
DEP_TERRAFORM_BUCKET="jerus-terraform-remote-state"
DEP_AWS_REGION="eu-west-1"
DEP_NAME="contact"
DEP_LAMBDA_NAME="contact"
DEP_LAMBDA_DESCRIPTION="Contact form processor."
DEP_API_ENDPOINT="contact"
DEP_FILE_NAME="lambda-contact-${DEP_VERSION}"

describe_local_variables()
{
YELLOW=$(tput setaf 6)
BOLD=$(tput bold)
RESET=$(tput sgr0)
printf "\n%s%sThe following local variables are set:%s%s" "$YELLOW" "$BOLD" "$RESET" "$YELLOW"
set | grep DEP | sed 's/^/    /'
printf "%s\n" "$RESET"
}

set_terraform_variables()
{
export TF_VAR_aws_profile_id=${AWS_PROFILE}
export TF_VAR_aws_account_id=${AWS_ACCOUNT_ID}
export TF_VAR_name=${DEP_NAME}
export TF_VAR_lambda_name=${DEP_LAMBDA_NAME}
export TF_VAR_lambda_description="${DEP_LAMBDA_DESCRIPTION}"
export TF_VAR_apigw=${DEP_APIGW}
export TF_VAR_api_endpoint=${DEP_API_ENDPOINT}
export TF_VAR_env="${DEP_ENVIRON}"
export TF_VAR_release="${DEP_VERSION}"
describe_variables
}
describe_local_variables

cleanup_config()
{
 if [ -f ".prod" ]; then
    rm prod.config
    rm prod.tfvars
fi
if [ -f ".test" ]; then
    rm test.config
    rm test.tfvars
fi
}

delete_KILLME_if_exits()
{
    if [ -f "${DEP_KILLME}" ]; then
        rm -f "${DEP_KILLME}"
    fi
}

check_environment()
{

    DEP_ENVIRON_CAPS=$(echo "${DEP_ENVIRON}" | tr '[:lower:]' '[:upper:]')

    if [ "${DEP_ENVIRON_CAPS}" = "PROD" ]; then
        DEP_APIGW="jerus-website"
        AWS_PROFILE="jerus-prod-tf"
        export AWS_PROFILE="jerus-prod-tf"
    else
        DEP_APIGW="website-test"
        AWS_PROFILE="jerus-dev-tf"
        export AWS_PROFILE="jerus-dev-tf"
    fi

    DEP_ENVIRON_FILE=.$(echo "${DEP_ENVIRON}" | tr '[:lower:]' '[:upper:]')
    if [ ! -f "$DEP_ENVIRON_FILE" ]; then 
        rm -rf .terraform
        if [ "$DEP_ENVIRON_FILE" = ".test" ]; then
            DEP_KILLME=".prod"
        else DEP_KILLME=".test"
        fi
        delete_KILLME_if_exits
        touch "$DEP_ENVIRON_FILE"
    fi
}

check_aws_profile_set()
{
if [ -z "${AWS_PROFILE}" ]; then
    printf "AWS_PROFILE not set"
    exit 1
fi
AWS_ACCOUNT_ID=$(aws sts get-caller-identity | jq -r ".Account")
}

describe_variables()
{
YELLOW=$(tput setaf 3)
BOLD=$(tput bold)
RESET=$(tput sgr0)
printf "\n%s%sThe following terraform variables are set:%s%s" "${YELLOW}" "${BOLD}" "${RESET}" "${YELLOW}"
env | grep TF_VAR | sed 's/^/    /'
printf "%s\n" "${RESET}"
}

create_config()
{
cat > "${DEP_ENVIRON}".config << EOF
key="${DEP_ENVIRON_CAPS}/${DEP_FILE_NAME}.tfstate"
bucket="${DEP_TERRAFORM_BUCKET}"
region="${DEP_AWS_REGION}"
EOF
}

create_tfvars()
{
cat > "${DEP_ENVIRON}".tfvars << EOF
region              = "${DEP_AWS_REGION}"
remote_state_key    = "${DEP_ENVIRON_CAPS}/${DEP_FILE_NAME}.tfstate"
remote_state_bucket = "${DEP_TERRAFORM_BUCKET}"
EOF
}

cmd_help()
{
printf "USAGAE: deploy.sh <cmd> [no-auto]\n"
printf "COMMANDS"
printf "  init        Re-initialise terraform by removing .terraform and running terraform init."
printf "  validate    Run terraform init and terraform validate."
printf "  plan        Run terraform init and terraform plan."
printf "  deploy      Run terraform init and terraform apply to deploy changes in AWS"
printf "  build       Build code and deploy"
printf "  destroy     Run terraform init and terraform destroy decomission AWS infrastructure\n"
printf "  "
printf "FLAGS"
printf "  no-auto     removes automatic application of terraform apply and terraform destroy\n"
printf "NOTES"
printf "On script changes:" | sed 's/^/  /'
printf "* run init to ensure all providers correctly configured" | sed 's/^/    /'
printf "* run validate to validate the script." | sed 's/^/    /'
printf "* run plan confirm the expected  impact." | sed 's/^/    /'
printf "* run deploy  no-auto flag to find errors when change on AWS is attempted.\n" | sed 's/^/    /'
printf "Once scripts have been validated they can be run by automation and applied automatically." | sed 's/^/  /'
}

cmd_init()
{
check_aws_profile_set
create_config
set_terraform_variables
rm -rf .terraform
printf 'terraform init -backend-config="%s.config"' "${DEP_ENVIRON}"
terraform init -backend-config="${DEP_ENVIRON}.config"
}

cmd_plan()
{
create_config
check_aws_profile_set
create_tfvars
set_terraform_variables
terraform init -backend-config="${DEP_ENVIRON}.config"
terraform plan -var-file="${DEP_ENVIRON}.tfvars"
}

cmd_validate()
{
create_config
check_aws_profile_set
create_tfvars
set_terraform_variables
terraform init -backend-config="${DEP_ENVIRON}.config"
terraform validate
}

cmd_import()
{
create_config
check_aws_profile_set
create_tfvars
set_terraform_variables
terraform init -backend-config="${DEP_ENVIRON}.config"
terraform import -var-file="${DEP_ENVIRON}.tfvars"  aws_lambda_permission.apigw_lambda contact/AllowExecutionFromAPIGateway
terraform import -var-file="${DEP_ENVIRON}.tfvars"  aws_lambda_function.lambda contact
}

cmd_deploy()
{
create_config
check_aws_profile_set
create_tfvars
set_terraform_variables
terraform init -backend-config="${DEP_ENVIRON}.config"
terraform apply -var-file="${DEP_ENVIRON}.tfvars" "${AUTO_APPROVE}"
}

cmd_destroy()
{
create_config
check_aws_profile_set
create_tfvars
set_terraform_variables
terraform init -backend-config="${DEP_ENVIRON}.config"
terraform destroy -var-file="${DEP_ENVIRON}.tfvars" "${AUTO_APPROVE}"
}

cmd_build()
{
RET=${PWD}
cd ..
./build.sh
cd "${RET}" || exit
cmd_deploy
}

CMD=$1
DEP_ENVIRON="test"

check_environment

if [ -z "${CMD}" ]; then
    CMD=help
elif [ ! ${CMD} =  "init" ] && [ ! "${CMD}" = "plan" ] && [ ! "${CMD}" = "validate" ] && [ ! "${CMD}" = "deploy" ] && [ ! "${CMD}" = "destroy"  ] && [ ! "${CMD}" = "build"  ] && [ ! "${CMD}" = "import" ]; then
    CMD=help
fi

if [ "${CMD}" = "init" ] || [ "${CMD}" = "validate" ] || [ "${CMD}" = "plan" ] || [ "${CMD}" = "deploy" ] || [ "${CMD}" = "build" ] || [ "${CMD}" = "destroy" ]; then
    if [ -z "${DEP_ENVIRON}" ]; then
        printf "Environment not specified. Please specify build environment."
        exit 1
    fi
fi

AUTO_APPROVE="-auto-approve"

if [ "$2" = "no-auto" ]; then
    AUTO_APPROVE=""
fi

if [ "${CMD}" = "help" ]; then cmd_help
elif [ "${CMD}" = "init" ]; then cmd_init
elif [ "${CMD}" = "plan" ]; then cmd_plan
elif [ "${CMD}" = "validate" ]; then cmd_validate
elif [ "${CMD}" = "import" ]; then cmd_import
elif [ "${CMD}" = "deploy" ]; then cmd_deploy
elif [ "${CMD}" = "upload" ]; then cmd_upload
elif [ "${CMD}" = "clean" ]; then cmd_clean
elif [ "${CMD}" = "destroy" ]; then cmd_destroy
elif [ "${CMD}" = "import" ]; then cmd_import
elif [ "${CMD}" = "build" ]; then cmd_build
fi


cleanup_config