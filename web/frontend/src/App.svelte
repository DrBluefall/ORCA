<script lang="ts">
    import PrismarineCoLogo from "./assets/pc_gear_v3.svg";

    let innerWidth = 0;
    let innerHeight = 0;
</script>

<svelte:head>
    <link
        rel="stylesheet"
        media="screen"
        href="https://fontlibrary.org//face/mononoki"
        type="text/css"
    />
</svelte:head>

<svelte:window bind:innerWidth bind:innerHeight />

<main>
    <nav>
        <p id="header-company">
            <img
                id="header-logo"
                src={PrismarineCoLogo}
                alt="The PrismarineCo logo: a hexagon surrounding a gear"
            />

            <span id="header-company-name">PrismarineCo.</span>
            <span id="header-company-name-long">The Prismarine Company</span>
        </p>
    </nav>
    <hr />
</main>

<style>
    #header-logo {
        height: 40px;
        vertical-align: middle;
        transform: translateY(-5%);
    }
    #header-company {
        font-family: "Mononoki", sans-serif;
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
