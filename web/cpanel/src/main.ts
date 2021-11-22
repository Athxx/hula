import App from './pages/App.svelte'
import Framework7 from 'framework7/lite-bundle'
import Framework7Svelte from "framework7-svelte"

import 'framework7/framework7-bundle.css'
import './assets/css/style.css'

Framework7.use(Framework7Svelte)

const app = new App({
	target: document.getElementById('app') as HTMLElement
})

export default app
