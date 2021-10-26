terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 3.61"
    }
    template = {
      source  = "hashicorp/template"
      version = ">= 2.0"
    }
  }
  required_version = ">= 0.13"
}
