# Static Email Generator (SEG)

SEG is a **developer-first** and **accessibility-first** email generator app. I
write this app to create, edit, test, and manage all emails using a template
engine. You can use it for generating single emails or you can create reusable
email templates for Mandrill, SendGrid, Zapier, etc.

You can also edit text-based email content for the visual-impaired humans or OCR
users, but if you forget that somehow, SEG will auto-generate it for you.

## Simple Usage

```
# generate emails
seg generate --src=templates/src/ \
             --dst=templates/dst/

# send a test mail to preview
seg send --template=templates/dst/wedding-invitation/ \
         --subject="You're invited to the wedding Ayse & Mehmet" \
         --email="john.doe@localhost"
```

## Immutable Variables

- subject
- body

## Email Folder Structure

All templates should have:

1. a subject.html,
2. and a body.html.

You don't need to create also a body_text.html but in case, SEG will
auto-generate it for making your emails more accessible for all human being.

## TO DO

[ ] minify css and html contents.
[ ] embed css files to the email template automatically.
[ ] support to embed markdown files to the email body as a variable.
[x] choose a template engine to render email contents.
[x] I also need a arg parser for CLI.
[x] create project skeleton.
