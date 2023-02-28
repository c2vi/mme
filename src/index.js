
import React from 'react'
import { useState } from 'react'
//import reactDom from "react-dom";
import { createRoot } from 'react-dom/client'
import Landing from './main'
//import { createContainer } from 'react-tracked';

let useItems = {}
let useItem = {}
let getItem = () => {}
let id = mize.id_to_render

class Test extends HTMLElement {
  connectedCallback() {}

  getItemCallback(item_raw) {
    this.item = item_raw

    const mountPoint = document.createElement('span')
    this.appendChild(mountPoint)

    this.old_items_len = 0

    this.do_not_send_update = {}
    this.id = mize.id_to_render
    this.items = {}

    useItem = (id, options = {}, initial = {}) => {
      if (id == undefined) {
        id = this.id
      }

      //the item rendered by this render
      const [state, setState] = useState(initial)

      //do some things only the first time
      if (state == initial) {
        this.items[id] = { item: state, setItem: setState, do_updates: true }
        this.do_not_send_update[id] = false
        let set_item = this.items[id].setItem

        //call get_item only the first time
        mize.get_item(id, (new_item) => {
			  pr("new_item", new_item)
          this.do_not_send_update[new_item["mize-id"]] = true
          this.items[new_item["mize-id"]].setItem(new_item)
        })

        if (!mize.update_callbacks[id]) {
          mize.update_callbacks[id] = []
        }

        mize.update_callbacks[id].push((update) => {
          this.do_not_send_update[update.now.id] = true
          this.items[update.now.id].raw = update.now
          set_item(update.now)
        })
      } else {
        if (!this.do_not_send_update[id]) {
          if (this.items[id].raw) {
            mize.update_item(this.items[id].item, state)
          }
        } else {
          this.do_not_send_update[id] = false
        }

        this.items[id].item = state
        this.items[id].setItem = setState
      }

      return [state, setState]
    }

    this.useItems_hooks = {}
    this.useItems_num = 0

    useItems = (ids, initial = {}) => {
      //if a single id was passed as a string, or no id was passed at all
      if (!ids) {
        ids = []
      } else if (!ids.length) {
        //ids is not an array
        ids = [ids]
      }

      initial._use_items_id = 0

      const [items, setItems] = useState(initial)

      let hook = {}

      hook.ids = ids
      if (items != initial) {
        hook = this.useItems_hooks[items._use_items_id]
      }

      //only the first time of every hook
      if (items == initial) {
        items._use_items_id = this.useItems_num
        this.useItems_num += 1

        this.useItems_hooks[items._use_items_id] = hook

        hook.raw_items = {}
        hook.do_not_send_update = {}

        //get those initial items, whoose ids are in ids
        let items_gotten = {}

        for (const id of hook.ids) {
          mize.get_item(id, (new_item) => {
            items_gotten[id] = new_item

            //if we have gotten all items, then call setItems
            if (Object.keys(items_gotten).length == hook.ids.length) {
              setItems((prev) => {
                let now = { ...prev }
                for (const id of hook.ids) {
                  hook.raw_items[id] = items_gotten[id]
                  hook.do_not_send_update[id] = true
                  now[id] = items_gotten[id].get_parsed()
                }
                return now
              })
            }
          })

          //register update_callbacks for all those items
          if (!mize.update_callbacks[id]) {
            mize.update_callbacks[id] = []
          }
          mize.update_callbacks[id].push((update) => {
            hook.do_not_send_update[update.now.id] = true
            hook.raw_items[update.now.id] = update.now
            setItems((prev) => {
              let now = { ...prev }
              now[id] = update.now.get_parsed()
              return now
            })
          })
        }
      } else {
        window.tmp = items
        for (const id of hook.ids) {
          if (!hook.do_not_send_update[id]) {
            if (hook.raw_items[id]) {
              hook.raw_items[id].update(items[id])
            } else {
            }
          } else {
            hook.do_not_send_update[id] = false
          }
        }
      }

      //hook.setItems = setItems
      //hook.initial_items_gotten = false

      const addItems = (ids) => {
        //if a single id was passed as a string
        if (typeof ids === 'string') {
          ids = [ids]
        }

        ids = ids.filter((id_ids) => {
          let id = hook.ids.find((id_hook) => id_hook === id_ids)
          return !id
        })

        hook.ids = [...hook.ids, ...ids]

        let items_gotten = {}

        for (const id of ids) {
          mize.get_item(id, (new_item) => {
            items_gotten[id] = new_item

            //if we have gotten all items, then call setItems
            if (Object.keys(items_gotten).length == ids.length) {
              setItems((prev) => {
                let now = { ...prev }
                for (const id of ids) {
                  hook.raw_items[id] = items_gotten[id]
                  now[id] = items_gotten[id].get_parsed()
                  hook.do_not_send_update[id] = true
                }
                return now
              })
            }

            // register update_callbacks for those newly gotten items
            if (!mize.update_callbacks[id]) {
              mize.update_callbacks[id] = []
            }
            mize.update_callbacks[id].push((update) => {
              hook.do_not_send_update[update.now.id] = true
              hook.raw_items[update.now.id] = update.now
              setItems((prev) => {
                let now = { ...prev }
                now[id] = update.now.get_parsed()
                return now
              })
            })
          })
        }
      }

      this.useItems_hooks[initial.__proto__.use_items_id] = hook
      delete items._use_items_id
      return [items, setItems, addItems]
    }

    //rendering to the webcomponent
    const root = createRoot(mountPoint) // createRoot(container!) if you use TypeScript

    root.render(
      //<ItemProvider>
      <Landing />
      //</ItemProvider>
    )
  }

  updateCallback(update) {
    //this.item = update.now
    //this.setItems((prev) => {
    //let new_state = prev
    //new_state[update.now.id] = update.now.get_parsed()
    //return new_state
    //})

    if (this.items[update.now.id]) {
      this.item_raw = update.now
      this.items[update.now.id].raw = update.now
      this.do_not_send_update[update.now.id] = true
      this.items[update.now.id].setItem((prev) => ({
        ...prev,
        ...update.now.get_parsed(),
      }))
    }
  }
}

export { getItem, useItems, useItem, id }
mize.defineRender(Test)
