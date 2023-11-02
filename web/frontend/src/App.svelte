<script lang="ts">
    import PrismarineCoLogo from "./assets/pc_gear_v3.svg";
    import Login from "./components/Login.svelte";
    import Signup from "./components/Signup.svelte";
    import Router from "svelte-spa-router";

    const routes = {
        "/signin": Login,
        "/signup": Signup,
    };
</script>

<main>
    <nav>
        <p id="header-company">
            <img
                id="header-logo"
                src={PrismarineCoLogo}
                alt="The PrismarineCo logo: a hexagon surrounding a gear" />

            <span id="header-company-name">PrismarineCo.</span>
            <span id="header-company-name-long">The Prismarine Company</span>
            |
            <a href="/#/signin">Sign In</a>
        </p>
    </nav>
    <hr />
    <div>
        <Router {routes} />
    </div>
</main>

<style>
    #header-logo {
        height: 40px;
        vertical-align: middle;
        transform: translateY(-5%);
    }

    #header-company {
        font-size: 30px;
    }

    @media (width > 700px) or (width < 300px) {
        #header-company-name {
            display: none;
        }
    }

    @media (width <= 700px) {
        #header-company-name-long {
            display: none;
        }
    }
</style>
