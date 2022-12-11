# https://maximillianxavier.medium.com/solving-cors-problem-on-local-development-with-docker-4d4a25cd8cfe
FROM nginx:alpine

WORKDIR /etc/nginx
# ./nginx.conf ./conf.d/default.conf
COPY . . 
EXPOSE 80
ENTRYPOINT [ "nginx" ]
CMD [ "-g", "daemon off;" ]