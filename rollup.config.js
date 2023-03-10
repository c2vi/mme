import { spawn } from 'child_process';
import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import terser from '@rollup/plugin-terser';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
//import css from 'rollup-plugin-css-only';
import css from 'rollup-plugin-postcss';

const production = !process.env.ROLLUP_WATCH;

function serve() {
	let server;

	function toExit() {
		if (server) server.kill(0);
	}

	return {
		writeBundle() {
			if (server) return;
			server = spawn('npm', ['run', 'start', '--', '--dev'], {
				stdio: ['ignore', 'inherit', 'inherit'],
				shell: true
			});

			process.on('SIGTERM', toExit);
			process.on('exit', toExit);
		}
	};
}

export default [{
  input: "src/main-folder.js",
  output: {
    sourcemap: !production,
    format: 'iife',
    name: 'app',
    // We output it to public. This way, our svelte kit
    // app will also host the web components.
    file: 'public/main-folder.js',
  },

 plugins: [
		svelte({
			emitCss: false,
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production,
        		//customElement: true,
				//accessors: true, //should be true by default
			},
			//preprocess: sveltePreprocess({
			  //sourceMap: !production,
			  //postcss: true,
			//}),
		}),
		// we'll extract any component CSS out into
		// a separate file - better for performance
		//css({ output: 'bundle.css' }),

		// If you have external dependencies installed from
		// npm, you'll most likely need these plugins. In
		// some cases you'll need additional configuration -
		// consult the documentation for details:
		// https://github.com/rollup/plugins/tree/master/packages/commonjs
		resolve({
			browser: true,
			dedupe: ['svelte'],
			exportConditions: ['svelte']
		}),
		commonjs(),

		// In dev mode, call `npm run start` once
		// the bundle has been generated
		//!production && serve(),

		// Watch the `public` directory and refresh the
		// browser on changes when not in production
		//!production && livereload('public'),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		//production && terser()
	],
	watch: {
		clearScreen: false
	}
},
{
  input: "src/top-bar.js",
  output: {
    sourcemap: !production,
    format: 'iife',
    name: 'app',
    // We output it to public. This way, our svelte kit
    // app will also host the web components.
    file: 'public/top-bar.js',
  },

 plugins: [
		svelte({
			emitCss: false,
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production,
        		//customElement: true,
				//accessors: true, //should be true by default
			},
			//preprocess: sveltePreprocess({
			  //sourceMap: !production,
			  //postcss: true,
			//}),
		}),
		// we'll extract any component CSS out into
		// a separate file - better for performance
		//css({ output: 'bundle.css' }),

		// If you have external dependencies installed from
		// npm, you'll most likely need these plugins. In
		// some cases you'll need additional configuration -
		// consult the documentation for details:
		// https://github.com/rollup/plugins/tree/master/packages/commonjs
		resolve({
			browser: true,
			dedupe: ['svelte'],
			exportConditions: ['svelte']
		}),
		commonjs(),

		// In dev mode, call `npm run start` once
		// the bundle has been generated
		//!production && serve(),

		// Watch the `public` directory and refresh the
		// browser on changes when not in production
		//!production && livereload('public'),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		//production && terser()
	],
	watch: {
		clearScreen: false
	}
}
];
