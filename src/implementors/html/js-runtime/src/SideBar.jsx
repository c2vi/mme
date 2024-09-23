import React, { useState } from 'react';

const SideBar = ({ tabs }) => {
  const [activeTab, setActiveTab] = useState(0);
  const [activeGroup, setActiveGroup] = useState(0);

  return (
   	<div className="w-64 h-screen bg-gray-100 overflow-scroll p-1 flex flex-col">
      	<div className="flex-grow overflow-y-scroll pr-2 mb-4">
        		<div className="space-y-2 overflow-y-scroll">
	  				{tabs.map((tabgroup, group_index) => (
	  					<TabSection group_index={group_index} key={group_index} tabs={tabgroup} activeTab={activeTab} setActiveTab={setActiveTab} setActiveGroup={setActiveGroup} activeGroup={activeGroup} />
					))}
        		</div>
      	</div>
   	</div>
  );
};

const TabSection = ({ tabs, activeTab, setActiveTab, group_index, activeGroup, setActiveGroup }) => {
	console.log("activeTab: ", activeTab)
	return (
		<div className="pt-4">
			<div className="text-left object-left left w-max m-1">
			GroupName
			</div>
			{tabs.map((tab, index) => (
				<button
					key={index}
					className={`w-full text-left px-2 py-2 pl-8 rounded-md transition-colors duration-200 ${
					activeTab === index && activeGroup === group_index
						? 'bg-blue-500 text-white'
						: 'text-gray-700 hover:bg-gray-200'
					}`}
					onClick={() => {
						setActiveGroup(group_index)
						setActiveTab(index)
					}}
				>
				{tab}
				</button>
			))}
		</div>
	)
}

export default SideBar;
