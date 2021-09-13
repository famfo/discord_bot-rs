/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_export]
macro_rules! twilight_embed {
    (
        $title: expr,
        $description: expr
    ) => {
        EmbedBuilder::new()
            .color(0x16990a)
            .title($title)
            .description($description)
            .footer(
                EmbedFooterBuilder::new("Bot coded in rust by famfo.").icon_url(ImageSource::url(
                    "https://avatars.githubusercontent.com/u/44938471",
                )?),
            )
            .build()?
    };
}
