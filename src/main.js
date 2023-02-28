import "./index.css"
import React from "react";
import { useItem } from "./index";
import {SideBar} from "./side-bar"
import {TopBar} from "./top-bar"
import {FolderMain} from "./folder-main"
import {FolderJson} from "./folder-json"

const Render = () =>{
	const [item, setItem] = useItem();
	pr(item)

	function update_cards(){
		setItem((prev) => ({...prev, cards_of_player: prev.cards_of_player + ["green_5"]}))
	}

		//<div className="absolute bg-red-500 rounded-md h-50 w-20 left-50 top-30">
			//<SideBar/>
		//</div>
	
   return <>
		<div className="absolute border-4 border-gray-500 rounded-md h-10 top-5 left-5 right-5">
			<TopBar/>
		</div>


		<div className="absolute bg-blue-500 rounded-md bottom-5 right-5 top-20 left-5">
			<FolderMain/>
		</div>
		</>
}

export default Render
