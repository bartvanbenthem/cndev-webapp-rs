# cndev-webapp-rs

Project template for creating server rendered web applications in Rust with Actix-web. Instead of a relational database, file storage is used for dynamic content to keep hosting costs at a minumum. 

### HealthCheck
```bash

# health check
curl -v -X GET \
  http://127.0.0.1:8000/health_check

```

### install and start MailHog
```bash

# On Docker
docker run -d -p 1025:1025 -p 8025:8025 mailhog/mailhog

# On Linux
mkdir ~/mailhog/
cd ~/mailhog
go mod init mailhog
go get github.com/mailhog/MailHog
go install github.com/mailhog/MailHog
~/go/bin/MailHog

```
