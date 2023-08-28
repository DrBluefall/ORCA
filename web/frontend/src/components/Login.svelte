<script lang="ts">
    let email = "";
    let password = "";

    async function signin(_e: SubmitEvent) {
        let response = await fetch("/api/signin", {
            method: "POST",
            mode: "same-origin",
            body: JSON.stringify({ email, password }),
        });

        if (response.status != 200) {
            console.log(JSON.stringify(response));
        }
    }
</script>

<div class="form">
    <form
        on:submit|preventDefault={async (e) => {
            await signin(e);
        }}
    >
        <label for="email">Email</label><br/>
        <input
            id="email"
            class="input-field"
            type="email"
            required
            name="email"
            autocomplete="username"
            bind:value={email}
            placeholder="user@example.com"
        /><br/>
        <label for="password">Password</label><br/>
        <input
            id="password"
            class="input-field"
            bind:value={password}
            type="password"
            placeholder="doNotUseThisPassword"
        /><br/>
        <div id="signin_button">
        <button type="submit">Sign Up!</button>
        </div>
    </form>
</div>

<style>
 * { box-sizing: border-box; }
 div.form {
     display: block;
     text-align: center;
 }
 form {
     display: flex;
     align-items: center;
     flex-direction: column;
     box-sizing: border-box;
 }
 #signin_button {
     width: 100%;
     display: block;
     padding: 10px;
 }
</style>
