# Simple Crawler Web
A very simple crawler web app
## Quickstart
1. Build container
    ```shell
    docker build -t simple-crawler-web .
    ```
2. Run container
    ```shell
    docker run -p 3030:3030 simple-crawler-web
   ```
## Endpoints
### Crawl
```
http://localhost:3030/crawl/?url=${URL}
```
### List
```
http://localhost:3030/list/?url=${URL}
```
### Count
```
http://localhost:3030/count/?url=${URL}
```
