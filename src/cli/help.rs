pub const BEFORE: &str = color_print::cstr!(
    r"<bold>   ___          __    __    __
  / _ ) __ __  / /   / /   / / ___
 / _  |/ // / / _ \ / _ \ / / / -_)
/____/ \_,_/ /_.__//_.__//_/  \__/</bold>"
);

pub const AFTER: &str = color_print::cstr!(
    "<bold><underline>Examples:</underline></bold>

  Start the socks5 proxy server, listening on '0.0.0.0:1080'

    <bold>./bubble --socks5</bold>

  Start the socks5 proxy server, listening on '127.0.0.1:9999'

    <bold>./bubble --socks5 --socks5-ip=127.0.0.1 --socks5-port=9999</bold>

  Start the http proxy server, listening on '0.0.0.0:1081'

    <bold>./bubble --http</bold>

  Start the http proxy server, listening on '0.0.0.0:9999'

    <bold>./bubble --http --http-port=9999</bold>

  Start both the socks5 and http proxy servers simultaneously, listening on their default addresses

    <bold>./bubble --socks5 --http\n</bold>
"
);
