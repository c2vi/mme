const tailwindcss = require('tailwindcss');
const autoprefixer = require('autoprefixer');

module.exports = {
	    plugins: [
			 tailwindcss('./tailwind.config.js'),
			 autoprefixer,
    		'postcss-preset-env',
		 ],
};

//module.exports = {
  //plugins: [
    //tailwindcss
  //],
//};

