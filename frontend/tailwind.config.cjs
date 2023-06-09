const colors = require('tailwindcss/colors');
const defaultTheme = require('tailwindcss/defaultTheme');
/** @type {import('tailwindcss').Config}*/
module.exports = {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}',
		require('path').join(require.resolve(
			'@skeletonlabs/skeleton'),
			'../**/*.{html,js,svelte,ts}'
		)
	],
	theme: {
		extend: {},
		// fontFamily: {
		// 	sans: [...defaultTheme.fontFamily.sans]
		// },
	},
	plugins: [
		require('@tailwindcss/forms'), ,
		...require('@skeletonlabs/skeleton/tailwind/skeleton.cjs')()]
};

module.exports = config;
