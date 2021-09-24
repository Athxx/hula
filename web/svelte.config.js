import svePreprocess from 'svelte-preprocess'

export default {
  preprocess: svePreprocess(),
  kit: {
    // hydrate the <div id="svelte"> element in src/app.html , https://kit.svelte.dev/docs#configuration
    target: '#svelte'
  }
}
