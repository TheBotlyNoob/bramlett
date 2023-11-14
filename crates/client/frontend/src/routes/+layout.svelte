<script>
	import '@fontsource/poppins';
	import '$lib/app.css';

    import { loadErrorMessages, loadDevMessages } from "@apollo/client/dev";
	import { dev } from '$app/environment';


    if (dev) {
        loadDevMessages();
        loadErrorMessages();
    }
</script>

<slot />
