<script lang='ts'>
	import Feeback from '@/pages/Home/Feeback.svelte';
	import ErrNotFound from '@/pages/Home/ErrNotFound.svelte';
	import {Router, Route} from 'svelte-routing';
	import Home from '@/pages/App/Home.svelte';
</script>

<Router>
	<Route component={Home} path='/'/>
	<Route component={Feeback} path='/fallback'/>
	<Route component={ErrNotFound}/>
</Router>
