import TopBar from './TopBar.svelte'

customElements.define(
 	"mize-mmejs-topbar",
  class extends HTMLElement {
    constructor() {
      super()

		 const webroot = "api/render/mize-mmejs-topbar/webroot"
  
      // Create the shadow root.
      this.shadow = this.attachShadow({ mode: 'open' })
	 }
  
	  getItemCallback(item){
      // Instantiate the Svelte Component
      this.element = new TopBar({
        // Tell it that it lives in the shadow root
        target: this.shadow,

        // Pass any props
        props: {
          // This is the place where you do any conversion between
          // the native string attributes and the types you expect
          // in your svelte components
			  item: item
        },
      })
    }

    disconnectedCallback() {
      // Destroy the Svelte component when this web component gets
      // disconnected. If this web component is expected to be moved
      // in the DOM, then you need to use `connectedCallback()` and
      // set it up again if necessary.
      this.element.destroy()
    }
  }
)
