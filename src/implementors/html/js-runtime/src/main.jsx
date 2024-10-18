import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'
import './index.css'
import { MmeJs } from 'mme'
import { comandr_ui_init } from '/home/me/work/dr-comandr/src/ui/react/src/main.jsx';
import { JsInstance } from '@c2vi/mize'

//console.log("wasm_bindgen", wasm_bindgen)

window.mme = new MmeJs()

window.mize = new JsInstance()


comandr_ui_init()


try {
	window.comandr.fns.main_show = function() {
		let input = document.getElementsByClassName("comandr-main")[0]
		input.focus()
	}
} catch (e) {
	console.log("comandr set main_show fn failed")
}


// render mme-js into the element with id root

let el = document.getElementById('root')

window.mme.presenters.mme_js(el)

