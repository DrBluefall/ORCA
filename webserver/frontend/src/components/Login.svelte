<script lang="ts">
    import { getCookie, removeCookie } from "typescript-cookie";
    import { current_user } from "../lib/userstore";

    let cookie = getCookie("discord_user");
    if (cookie !== undefined) {
        $current_user = JSON.parse(cookie);
    }

    function on_logout() {
        removeCookie("discord_user");
        $current_user = null;
    }
</script>

{@debug current_user}

<div id="login">
    {#if $current_user !== null}
        Hello, {$current_user.username}.
        <a href="/" on:click={on_logout}>Logout</a>
    {:else}
        <a href="/login">Login</a>
    {/if}
</div>

<style>
 #login {
   float: left;
 }
</style>
