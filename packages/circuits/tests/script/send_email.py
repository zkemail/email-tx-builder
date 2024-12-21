import smtplib
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
import os
import json
import glob
from dotenv import load_dotenv
import re


def send_html_email(
    sender_email, sender_password, recipient_email, subject, message, sender_name
):
    # Create a multipart message
    msg = MIMEMultipart("alternative")
    if sender_name != None:
        msg["From"] = f"{sender_name} <{sender_email}>"
    else:
        msg["From"] = sender_email
    msg["To"] = recipient_email
    msg["Subject"] = subject
    html_content = f"""
    <html>
    <body>
        <h1>Hello!</h1>
        <p>This is a test email with a basic HTML body.</p>
        <div style="display: none;">{message}</div>
        <p><b>Thank you!</b></p>
    </body>
    </html>
    """

    # Attach the HTML body to the email
    msg.attach(MIMEText(html_content, "html"))

    try:
        # Connect to the Gmail SMTP server
        server = smtplib.SMTP("smtp.gmail.com", 587)
        server.starttls()  # Upgrade the connection to a secure encrypted SSL/TLS connection
        server.login(sender_email, sender_password)

        # Send the email
        server.sendmail(sender_email, recipient_email, msg.as_string())

        print("Email sent successfully!")
    except Exception as e:
        print(f"Failed to send email: {e}")
    finally:
        server.quit()


if __name__ == "__main__":
    # Replace with your details
    load_dotenv()
    sender_email = os.getenv("SENDER_EMAIL")
    sender_password = os.getenv("SENDER_PASSWORD")
    recipient_email = os.getenv("RECIPIENT_EMAIL")
    lower_id = os.getenv("LOWER_ID")
    upper_id = os.getenv("UPPER_ID")
    print(f"Sender email: {sender_email}")
    script_dir = os.path.dirname(os.path.abspath(__file__))
    json_files = glob.glob(os.path.join(script_dir, "../templates/*.json"))
    for json_file in json_files:
        match = re.search(r"email_auth_test(\d+)\.json", os.path.basename(json_file))
        file_number = int(match.group(1))
        if lower_id != None and file_number < int(lower_id):
            continue
        if upper_id != None and file_number > int(upper_id):
            continue
        with open(json_file, "r") as file:
            data = json.load(file)
            subject = data.get("subject")
            message = data.get("body")
            sender_name = data.get("senderName")
            print(f"Sending email with {subject}; {message}; {sender_name}")
            send_html_email(
                sender_email,
                sender_password,
                recipient_email,
                subject,
                message,
                sender_name,
            )
