# About 
Make your life easier with Tiktok uploader bot in Telegram.
Send the link, wait for the response, forward the video to your friends!
No watermarks as well.

## Linux and Docker support
1. Install [Docker](https://www.docker.com/) on your main OS.
2. Build docker image `docker build -t telegram-bot .`
3. Run the container `docker run --restart=unless-stopped -d -e TELOXIDE_TOKEN='your bot token here' telegram-bot`