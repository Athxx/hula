<script lang="ts">
	import Routes from '../components/Routes.svelte';
	import {App, View} from 'framework7-svelte';
</script>

<App
	id='com.website'
	name='X'
	theme='auto'
	statusbar={{
        iosOverlaysWebView: true,
        androidOverlaysWebView: true,
    }}
>
	<View>
		<Routes/>
	</View>
</App>
