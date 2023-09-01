<script lang="ts">
    let email = "";
    let password = "";
    let username = "";

    async function auth() {
        let response = await fetch("/api/site/signup", {
            method: "POST",
            mode: "same-origin",
            body: JSON.stringify({ email, password, username }),
            headers: { "Content-Type": "application/json" },
        });

        if (response.status != 200) {
            console.log(JSON.stringify(response));
        } else {
            // log in on successful auth
            window.location.replace("/");
        }
    }
</script>

<div class="form">
    <h1>Let's get you in!</h1>
    <form>
        <label for="username">Username</label>
        <input
            id="username"
            class="input-field"
            type="text"
            required
            name="username"
            bind:value={username}
            placeholder="Squidbeak"
        />
        <label for="email">Email</label>
        <input
            id="email"
            class="input-field"
            type="email"
            required
            name="email"
            bind:value={email}
            placeholder="user@example.com"
        /><br />
        <label for="password">Password</label>
        <input
            id="password"
            class="input-field"
            bind:value={password}
            type="password"
            placeholder="doNotUseThisPassword"
        /><br />
        <div id="signin_button">
            <button on:click|preventDefault={async () => await auth()}
                >Sign Up</button
            >
            <br />
        </div>
        <a href="/#/signin">Already have an account?</a>
    </form>
</div>

<style>
    * {
        box-sizing: border-box;
    }
    div.form {
        display: block;
        text-align: center;
    }
    .input-field {
        padding: 5px;
        margin: 5px;
    }
    form {
        display: flex;
        align-items: center;
        flex-direction: column;
        box-sizing: border-box;
    }
    button {
        font-family: inherit;
        font-size: 1em;
    }
    #signin_button {
        width: 100%;
        display: block;
        padding: 10px;
    }
</style>
