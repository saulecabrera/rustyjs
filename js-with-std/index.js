var encode = base64.encode;

var Shopify = {
  main: performEncoding,
};


function performEncoding() {
  return encode("foo");
}
