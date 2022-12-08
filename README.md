# The Omnicient Recording Computer of Alterna

Now with 100% more oxidation!

## Running the bot

1. Prepare a MySQL database to run the bot against.
1. Copy `orcaconf.example.toml` to `orcaconf.toml`. Configure it as desired.
2. Run `cargo run`. The bot will automatically run migrations against the
   database specified in `orcaconf.toml` if they haven't been run already.
3. ...
4. Profit!

## Roadmap

### STAGE 0: THE MOST BASIC FUNCTIONALITY
- [ ] Basic profile details, parity with summary screen in SplatNet 3
  - [X] IGN & discriminator
  - [x] Level
  - [ ] Loadout (*maaaaaaybe* multiple? consider `/dossier fits` command in future)
  - [x] Rank Info
    - [x] Anarchy Battle (Current/All-Time)
    - [x] X Battle Statistics (only show if Anarchy Battle stats are > S+0)
  - [x] Turf Inked
  - [x] Total Victories
  - [ ] Badge Count
  - [ ] Preferred Weapons
  - [ ] Titles\*
- [ ] Profile editing website (because dossiers are a bit too complex to edit
      within Discord)

\*: Titles will be complicated. Mostly because of differences in languages.
  
### STAGE 1: QUALITY OF LIFE FEATURES
- [ ] Splatoon3.ink integration
- [ ] Localized Commands
  - possible i18n path? re: Fluent & `fluent-templates`

## License

> Copyright 2022 Alexander Bisono.
>
> O.R.C.A is free software: you can redistribute it and/or modify it under the
> terms of the GNU Affero General Public License as published by the Free
> Software Foundation, either version 3 of the License, or (at your option) any
> later version.
>
> O.R.C.A is distributed in the hope that it will be useful, but WITHOUT ANY
> WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
> A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
> details.
>
> You should have received a copy of the GNU Affero General Public License along
> with O.R.C.A. If not, see <https://www.gnu.org/licenses/>.

