provider "aws" {
  region  = var.region
  profile = var.aws_profile_id
}

terraform {
  backend "s3" {}
}

resource "aws_ses_template" "contact_notification" {
  name    = var.name
  subject = "Thank you for your interest in Jerus Data Protection"
  html    = "<p>Hi {{name}},</p><br><p>Your contact details have been logged and we will respond in the coming days.</p><br><p>Alternatively you can <a href='https://app-de.cronofy.com/pls/pyd4IrHA'>schedule an online meeting with me</a></p><br><p>Jeremiah Russell</p><p>Managing Director</p><br><p>“Protect your business by caring for personal data”</p><br><p style='font-size: 80%; font-weight: bold'>Jerus Data Protection Ltd registered in Dublin, Ireland No. 620949 | Registered Office: 20 Harcourt Street, Dublin, D02 H364, Ireland</p><br><p style='font-size: 60%'>This e-mail, including attachments, may contain confidential and/or legally privileged material for the sole use of the intended recipient.  If you are not the intended recipient (or authorized to receive for the recipient) please contact the sender by reply e-mail and delete all copies of this message.</p>"
  text    = "Hi {{name}},\r\n\r\nYour contact details have been logged and we will respond in the coming days.\r\n\r\nAlternatively you can schedule an online meeting with me: https://app-de.cronofy.com/pls/pyd4IrHA\r\n\r\nJeremiah Russell\r\nManaging Director\r\n\r\n“Protect your business by caring for personal data”\r\n\r\nJerus Data Protection Ltd registered in Dublin, Ireland No. 620949 | Registered Office: 20 Harcourt Street, Dublin, D02 H364, Ireland\r\n\r\nThis e-mail, including attachments, may contain confidential and/or legally privileged material for the sole use of the intended recipient.  If you are not the intended recipient (or authorized to receive for the recipient) please contact the sender by reply e-mail and delete all copies of this message.\r\n"
}
