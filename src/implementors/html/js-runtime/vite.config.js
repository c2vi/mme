import { defineConfig } from 'vite'
//import react from '@vitejs/plugin-react'
//import { viteSingleFile } from "vite-plugin-singlefile"
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [ 
		//react(), 
		//viteSingleFile(), 
		wasm(), 
		topLevelAwait() 
	],
  /*
	resolve: {
		alias: [{ find: '@', replacement: '/src' }],
	},
   rollupOptions: {
   	external: [
      	"comandr_ui_react",
			"/home/me/work/mme-presenters/presenters/mme-js/src/main.jsx",
			//"@c2vi/mize/worker.js?url"
   	],
	},
	base: '/',
  */
})

