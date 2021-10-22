<script lang='ts'>
	import {Router, Route} from 'svelte-routing';
	import FallbackPage from '@/pages/FallbackPage.svelte';
	import HomePage from '@/pages/HomePage.svelte';
</script>

<Router>
	<Route component={HomePage} path='/'/>
	<Route component={FallbackPage} path='*'/>
</Router>
