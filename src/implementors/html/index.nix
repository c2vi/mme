{ mize_url, ... }: ''
<html>
  <head>
    <script src="${mize_url}"></script>
    <script>
      const { JsInstance } = wasm_bindgen;

      wasm_bindgen().then(() => {
        window.mize = new JsInstance()
        window.mize.load_module("mme")
      });

    </script>
  </head>
	<body>
  	hello world...............
  </body>  
</html>
''
