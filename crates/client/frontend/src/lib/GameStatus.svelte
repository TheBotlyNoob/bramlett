<script lang="ts">
	import { GraphQlGameStatusInner, type GamesQuery, DownloadGame, RunGame } from '$lib/gql';

	export let game: GamesQuery['games'][0];

	let progress = game.status.progress
		? Math.round((game.status.progress[0] / game.status.progress[1]) * 100)
		: NaN;
</script>

{#if game.status.status == GraphQlGameStatusInner.NotDownloaded}
	<button
		class="btn btn-primary btn-sm"
		on:click={() => {
			DownloadGame({
				variables: {
					game: game.id
				}
			});
		}}>Download</button
	>
{:else if game.status.status == GraphQlGameStatusInner.Downloading}
	<div class="radial-progress text-primary" style="--value: {progress}" role="progressbar">
		{progress}%
	</div>
{:else if game.status.status == GraphQlGameStatusInner.Installing}
	<div class="radial-progress text-secondary" style="--value: {progress}" role="progressbar">
		{progress}%
	</div>
{:else if game.status.status == GraphQlGameStatusInner.Ready}
	<button
		class="btn btn-secondary btn-sm"
		on:click={() => {
			RunGame({
				variables: {
					game: game.id
				}
			});
		}}>Play</button
	>
{:else if game.status.status == GraphQlGameStatusInner.Running}
	<span>Running...</span>
{/if}
