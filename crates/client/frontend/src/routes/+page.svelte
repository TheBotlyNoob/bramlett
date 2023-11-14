<script lang="ts">
	import { Games } from '$lib/gql';
	import GameStatus from '$lib/GameStatus.svelte';
	import { browser } from '$app/environment';
	import { readable } from 'svelte/store';

    $: games = browser ? Games({
        pollInterval: 3000
    }) : readable({
        data: {
            games: []
        },
        loading: true,
    }, () => {});

    $: console.log($games)
</script>

{#if $games.loading}
	<!-- maybe add loading animation? for now, it looks fine. -->
{:else}
	<div class="grid gap-4 grid-cols-6 lg:grid-cols-4">
		{#each $games?.data?.games || [] as game}
			<div class="card card-compact lg:w-72 w-36 bg-base-300 shadow-xl">
				<figure><img src={game.icon} alt={game.name} /></figure>
				<div class="card-body lg:flex lg:flex-row lg:justify-between lg:place-items-center">
					<h2 class="text-lg font-bold">{game.name}</h2>
					<GameStatus {game} />
				</div>
			</div>
		{/each}
	</div>
{/if}
