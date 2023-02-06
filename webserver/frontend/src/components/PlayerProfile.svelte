<script lang="ts">
    import { onMount } from "svelte";
    import { current_user } from "../lib/userstore";

    let pd_promise: Promise<any>;

    async function get_player_data() {
        if ($current_user === null) return null;
        const res = await fetch(`/api/profile/${$current_user.id}`);
        const text = await res.text();

        if (res.ok) {
            return JSON.parse(text);
        } else {
            throw new Error(text);
        }
    }

    onMount(() => {
        pd_promise = get_player_data();
    });

    current_user.subscribe((_) => {
        pd_promise = get_player_data();
    });
</script>

<div id="player_profile">
    {#await pd_promise}
        <p>Fetching profile data...</p>
    {:then player_data}
        {#if player_data !== null}
            <img
                style="vertical-align: top;"
                src="https://cdn.discordapp.com/avatars/{$current_user.id}/{$current_user.avatar}.png"
                alt="Avatar of Citizen '{player_data.ign}#{player_data.discriminator}'"
                id="user_picture"
            />
            <h1>
                <span class="user_info">
                    {player_data.ign}#{player_data.discriminator}
                </span>
            </h1>
            <sub class="user_info">Citizen #{$current_user.id}</sub>
        {/if}
    {:catch err}
        <p style="color: red">{err.message}</p>
    {/await}
</div>

<style>
    .user_info {
        font-family: "mononokiRegular", monospace;
        line-height: 1em;
    }

    #player_profile {
        display: inline-block;
    }
</style>
