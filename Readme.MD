# learn-otel

I am using `learn-otel` to help understand opentelemetry and how to incorporate in web applications, written in `rust`

# How this repo will evolve

Will be starting with the basics and slowing building it up. Thus I see that I will be current releases as I achieve a mini learning goal I set for myself.

# Notes

The sample uses jaeger, so run it locally using this command:

```
  docker run -d --name jaeger \
  -e COLLECTOR_ZIPKIN_HTTP_PORT=9411 \
  -p 5775:5775/udp \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 14268:14268 \
  -p 9411:9411 \
  jaegertracing/all-in-one:1.6
```

The logs will can be view in the jaeger web ui at http://localhost:16686.
