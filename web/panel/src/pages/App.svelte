<script lang="ts">
	import {App, View} from "framework7-svelte";
	import Routes from "@/components/Routes.svelte";
</script>

<App
	id='com.website'
	name='My App'
	statusbar={{
        iosOverlaysWebView: true,
        androidOverlaysWebView: false,
    }}
	theme='auto'
>
	<View>
		<Routes/>
	</View>
</App>
