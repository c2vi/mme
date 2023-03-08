import FolderMain from './Link.svelte'

customElements.define(
 	"mize-mmejs-mainfolder",
  class extends HTMLElement {
    constructor() {
      super()
  
      // Create the shadow root.
      this.shadow = this.attachShadow({ mode: 'open' })
	 }
  
	  getItemCallback(item){
      // Instantiate the Svelte Component
      this.element = new FolderMain({
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
