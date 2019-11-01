FROM gcr.io/distroless/static

COPY target/release/darkpools_publicapi_ws_pairs /bin/

CMD ["/bin/darkpools_publicapi_ws_pairs"]