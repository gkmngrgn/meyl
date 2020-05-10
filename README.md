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
- content (or body?)

## TO DO

[ ] choose a template engine to render email contents.
[x] create project skeleton.
