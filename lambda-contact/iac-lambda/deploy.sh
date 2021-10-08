#!/bin/sh
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
YELLOW=`tput setaf 6`
BOLD=`tput bold`
RESET=`tput sgr0`
echo "\n${YELLOW}${BOLD}The following local variables are set:${RESET}${YELLOW}"
set | grep DEP | sed 's/^/    /'
echo "${RESET}\n"
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
        rm -f ${DEP_KILLME}
    fi
}

check_environment()
{

    DEP_ENVIRON_CAPS=$(echo "${DEP_ENVIRON}" | tr a-z A-Z)

    if [ ${DEP_ENVIRON_CAPS} = "PROD" ]; then
        DEP_APIGW="jerus-website"
        AWS_PROFILE="jerus-prod-tf"
        export AWS_PROFILE="jerus-prod-tf"
    else
        DEP_APIGW="website-test"
        AWS_PROFILE="jerus-dev-tf"
        export AWS_PROFILE="jerus-dev-tf"
    fi

    DEP_ENVIRON_FILE=.$(echo "${DEP_ENVIRON}" | tr A-Z a-z)
    if [ ! -f "$DEP_ENVIRON_FILE" ]; then 
        rm -rf .terraform
        if [ "$DEP_ENVIRON_FILE" = ".test" ]; then
            DEP_KILLME=".prod"
        else DEP_KILLME=".test"
        fi
        delete_KILLME_if_exits
        touch $DEP_ENVIRON_FILE
    fi
}

check_aws_profile_set()
{
if [ -z "${AWS_PROFILE}" ]; then
    echo "AWS_PROFILE not set"
    exit 1
fi
AWS_ACCOUNT_ID=`aws sts get-caller-identity | jq -r ".Account"`
}

describe_variables()
{
YELLOW=`tput setaf 3`
BOLD=`tput bold`
RESET=`tput sgr0`
echo "\n${YELLOW}${BOLD}The following terraform variables are set:${RESET}${YELLOW}"
env | grep TF_VAR | sed 's/^/    /'
echo "${RESET}\n"
}

create_config()
{
cat > ${DEP_ENVIRON}.config << EOF
key="${DEP_ENVIRON_CAPS}/${DEP_FILE_NAME}.tfstate"
bucket="${DEP_TERRAFORM_BUCKET}"
region="${DEP_AWS_REGION}"
EOF
}

create_tfvars()
{
cat > ${DEP_ENVIRON}.tfvars << EOF
region              = "${DEP_AWS_REGION}"
remote_state_key    = "${DEP_ENVIRON_CAPS}/${DEP_FILE_NAME}.tfstate"
remote_state_bucket = "${DEP_TERRAFORM_BUCKET}"
EOF
}

cmd_help()
{
echo "USAGAE: deploy.sh <cmd> [environment] [no-auto]\n"
echo "COMMANDS"
echo "  init        Re-initialise terraform by removing .terraform and running terraform init."
echo "  validate    Run terraform init and terraform validate."
echo "  plan        Run terraform init and terraform plan."
echo "  deploy      Run terraform init and terraform apply to deploy changes in AWS"
echo "  build       Build code and deploy"
echo "  destroy     Run terraform init and terraform destroy decomission AWS infrastructure\n"
echo "  "
echo "FLAGS"
echo "  no-auto     removes automatic application of terraform apply and terraform destroy\n"
echo "NOTES"
echo "On script changes:" | sed 's/^/  /'
echo "* run init to ensure all providers correctly configured" | sed 's/^/    /'
echo "* run validate to validate the script." | sed 's/^/    /'
echo "* run plan confirm the expected  impact." | sed 's/^/    /'
echo "* run deploy  no-auto flag to find errors when change on AWS is attempted.\n" | sed 's/^/    /'
echo "Once scripts have been validated they can be run by automation and applied automatically." | sed 's/^/  /'
}

cmd_init()
{
check_aws_profile_set
create_config
set_terraform_variables
rm -rf .terraform
echo terraform init -backend-config="${DEP_ENVIRON}.config"
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
terraform apply -var-file="${DEP_ENVIRON}.tfvars" ${AUTO_APPROVE}
}

cmd_destroy()
{
create_config
check_aws_profile_set
create_tfvars
set_terraform_variables
terraform init -backend-config="${DEP_ENVIRON}.config"
terraform destroy -var-file="${DEP_ENVIRON}.tfvars" ${AUTO_APPROVE}
}

cmd_build()
{
RET=${PWD}
cd ..
./build.sh
cd ${RET}
cmd_deploy
}

CMD=$1
DEP_ENVIRON=$2

check_environment

if [ -z "${CMD}" ]; then 
    CMD=help
elif [ ! ${CMD} =  "init" -a ! "${CMD}" = "plan" -a ! "${CMD}" = "validate" -a ! "${CMD}" = "deploy" -a ! "${CMD}" = "destroy"  -a ! "${CMD}" = "build"  -a ! "${CMD}" = "import" ]; then
    CMD=help
fi

if [ "${CMD}" = "init" -o "${CMD}" = "validate" -o "${CMD}" = "plan" -o "${CMD}" = "deploy" -o "${CMD}" = "build" -o "${CMD}" = "destroy" ]; then
    if [ -z "${DEP_ENVIRON}" ]; then
        echo "Environment not specified. Please specify build environment."
        exit 1
    fi
fi

AUTO_APPROVE="-auto-approve"

if [ "$3" = "no-auto" ]; then
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