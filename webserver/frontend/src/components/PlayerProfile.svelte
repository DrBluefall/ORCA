<script lang="ts">
    import { current_user } from "../lib/userstore";

    let pd_promise: Promise<object>;

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

    current_user.subscribe((_) => {
        pd_promise = get_player_data();
    });
</script>

<div id="player_profile">
    {#await pd_promise}
        <p>Fetching profile data...</p>
    {:then player_data}
        {#if player_data !== null}
            <pre>
            {JSON.stringify(player_data)}
            </pre>
        {/if}
    {:catch err}
        <p style="color: red">{err.message}</p>
    {/await}
</div>

<style></style>
