import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
  server: {
    proxy: {
         '/api': {
           target: 'http://0.0.0.0:3031',
           changeOrigin: true,
           secure: false,      
           ws: true,
         },
        '/replay': {
             target: 'http://0.0.0.0:3030',
             changeOrigin: true,
             secure: false,      
             ws: true,
         },


     }
  }
});

