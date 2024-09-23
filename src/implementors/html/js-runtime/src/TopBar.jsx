import React, { useState } from 'react';
import { Terminal, Menu, Plus, Settings } from 'lucide-react';
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut,
} from "@/components/ui/command"


const ExplorerTopbar = () => {
	const [commands, setCommands] = useState({ term: "", commands: ["first", "2ond"]})

	function onChange(e) {
		//setCommands({ term: e, commands: mme.comandr_search(e)})
		setCommands({ term: e, commands: ["hi", "test"]})
	}
	/*
				{commands.commands.map((name) => (
					<CommandItem> {name} </ CommandItem>
				))}
	*/

  	return (
    <div className="flex items-center justify-between p-2 bg-gray-100 border-b border-gray-200">
      <div className="flex items-center">
        <Button variant="ghost" size="icon" className="mr-2" >
          <Menu className="h-4 w-4" />
        </Button>
        <h1 className="text-lg font-semibold">MiZe</h1>
      </div>
      <div className="flex-grow mx-20">
		<div className="relative">
   		<Terminal className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
	  		<Command>
         	<CommandInput
            	type="text"
            	placeholder = "Command"
            	className="pl-8 w-full comandr-main"
	  				value = { commands.term }
	  				onValueChange = { onChange }
          	/>
	  		</Command>
        </div>
      </div>
      <div className="flex items-center space-x-2">
        <Button variant="ghost" size="icon">
          <Plus className="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="icon">
          <Settings className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
};

export default ExplorerTopbar;
