<script lang='ts'>
	import Feedback from '../pages/Home/Feedback.svelte';
	import NotFound from '../pages/Home/NotFound.svelte';
	import {Router, Route} from 'svelte-routing';
	import Home from '../pages/Home/Home.svelte';
	let url = window.location.pathname;
</script>

<Router url='{url}'>
	<Route component={Home} path='/'/>
	<Route component={Feedback} path='/feedback'/>
	<Route component={NotFound}/>
</Router>

