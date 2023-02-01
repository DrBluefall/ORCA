<script lang="ts">
    import { getCookie, removeCookie } from "typescript-cookie";
    import { current_user } from "../lib/userstore";
    import { get } from "svelte/store";

    let cookie = getCookie("discord_user");
    if (cookie !== undefined) {
        current_user.update((_) => JSON.parse(cookie));
    }

    function on_logout() {
        removeCookie("discord_user");
        current_user.update((_) => null);
    }

    function on_login() {
        window.location.href = "/login";
    }
</script>

<main>
    {#if get(current_user) !== null}
        Hello, {get(current_user).username}.
        <button on:click={on_logout}>Logout</button>
    {:else}
        <button on:click={on_login}>Login</button>
    {/if}
</main>
