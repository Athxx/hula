<Page name="home">
	<Navbar title="Home Page"/>

	<Link to="/feedback">Feedback Page</Link>
	<Link to="/404">404 Page</Link>
</Page>
<script lang="ts">
	import {Page, Navbar} from 'framework7-svelte';
	import { Link } from "svelte-routing";
</script>

<style>
	/* Custom color theme properties */
	:global(:root) {
		--f7-theme-color: #F17010;
		--f7-theme-color-rgb: #ffffff;
		--f7-theme-color-shade: #cc5e0c;
		--f7-theme-color-tint: #f38737;
	}
</style>
