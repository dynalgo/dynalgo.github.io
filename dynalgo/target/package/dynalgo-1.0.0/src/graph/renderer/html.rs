pub struct Html {}

impl Html {
    pub fn render(title: &str, svg_content: String) -> String {
        let mut html = String::from(
            "
<!DOCTYPE html>
<html>
  <head>
    <meta charset=\"utf-8\" />
    <meta http-equiv=\"pragma\" content=\"no-cache\">
    <meta http-equiv=\"expires\" content=\"0\">
    <meta http-equiv=\"cache-control\" content=\"no-cache, must-revalidate\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">",
        );
        html.push_str(&format!("<title>Dynalgo - {}</title>", title));
        html.push_str(
            "
    <style>
        html,
        body {
            margin: 0;
            padding: 0;
            overflow: hidden;
        }
        #svg_dynalgo {
            position: fixed;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
            width: 100%;
            height: 100vh;
    }
    </style>
    <script>
        function pause(svg) {
            if (svg.animationsPaused()) {
                svg.unpauseAnimations();
            } else {
                svg.pauseAnimations();
            }
        }
    </script>
  </head>
  <body>",
        );
        html.push_str(&svg_content);
        html.push_str(
            "
  </body>
</html>",
        );

        html
    }
}
