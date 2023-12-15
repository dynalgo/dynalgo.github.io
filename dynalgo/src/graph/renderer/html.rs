pub struct Html {}

impl Html {
    pub fn render_flexbox(
        titles: &Vec<String>,
        title_idx: usize,
        file_names: &Vec<String>,
        contents: Vec<String>,
    ) -> String {
        let back_color = (255, 255, 255);
        let text_color = (0, 0, 139);
        let sepe_color = (211, 211, 211);

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
        html.push_str(&format!("<title>Dynalgo - {}</title>", titles[title_idx]));
        html.push_str(
            "
   <style>
		body {
        ",
        );
        html.push_str(&format!(
            "            background-color: rgb({}, {}, {});",
            back_color.0, back_color.1, back_color.2
        ));
        html.push_str(&format!(
            "            color: rgb({}, {}, {});",
            text_color.0, text_color.1, text_color.2
        ));
        html.push_str(
            "            
			margin: 0px;
			padding: 0px;
			display: flex;
		}

		:link {
        ",
        );
        html.push_str(&format!(
            "            color: rgb({}, {}, {});",
            text_color.0, text_color.1, text_color.2
        ));
        html.push_str(
            "      
		}",
        );

        if titles.len() > 1 {
            html.push_str(
                "
		nav {
		    font-size: 1em;
		    border: none;
		    display: flex;
		    flex-flow: column wrap;
		    padding: 0px;
		    margin: 0px;
		    min-width: 230px;
		}

		nav ul {
		    margin: 0px;
		    padding: 0px;
		    list-style: none;
		    display: flex;
		    flex-flow: column wrap;
		    justify-content: left;
		}

		nav a {
		    padding-top: 0.2em;
		    display: block;
		    text-align: left;
		    text-decoration: none;
        ",
            );
            html.push_str(&format!(
                "            color: rgb({}, {}, {});",
                text_color.0, text_color.1, text_color.2
            ));
            html.push_str(
                "  
		}

        li {
        ",
            );
            html.push_str(&format!(
                "            border-top: 1px solid rgb({}, {}, {});\n",
                back_color.0, back_color.1, back_color.2
            ));
            html.push_str(&format!(
                "            border-bottom: 1px solid rgb({}, {}, {});",
                back_color.0, back_color.1, back_color.2
            ));
            html.push_str(
                "            
        }

		li:hover{
        ",
            );
            html.push_str(&format!(
                "            border-top: 1px solid rgb({}, {}, {});\n",
                text_color.0, text_color.1, text_color.2
            ));
            html.push_str(&format!(
                "            border-bottom: 1px solid rgb({}, {}, {});",
                text_color.0, text_color.1, text_color.2
            ));
            html.push_str(
                "            
        }

		header {
		    display: flex;
		    padding: 0px;
		    margin: 0px;
		    justify-content: space-around;
        ",
            );
            html.push_str(&format!(
                "            border-bottom: 1px solid rgb({}, {}, {});",
                sepe_color.0, sepe_color.1, sepe_color.2
            ));
            html.push_str(
                "                        
		}",
            );
        }
        html.push_str(
            "
		section {
			display: flex;
			flex-flow: row nowrap;
			width: 100%;
			height: 100vh;
			padding: 0px;
			margin: 0px;
			justify-content: space-between;
		}

		article {
			display: flex;
			border: none;
			width: 50%;
			height: 100%;
			padding: 0px;
			margin: 0px;
			overflow: hidden;
        ",
        );
        html.push_str(&format!(
            "            border-left: 1px solid rgb({}, {}, {});",
            sepe_color.0, sepe_color.1, sepe_color.2
        ));
        html.push_str(
            "      
		}

		 svg text{
		  text-anchor: middle;
		  dominant-baseline: central;
		}

		@media screen and (max-width: 800px) {
			body {
				flex-direction: column;
				width: 100%;
			}

			section {
				border-left: none;
				flex-direction: column;
				width: 100%;
			}

			article {
        ",
        );
        html.push_str(&format!(
            "            border-top: 1px solid rgb({}, {}, {});",
            sepe_color.0, sepe_color.1, sepe_color.2
        ));
        html.push_str(
            "    
				border-left: none;
				width: 100%;
			}
",
        );

        if titles.len() > 1 {
            html.push_str(
                "
			nav {
		    	width: 100%;
			}

			nav ul {
				border-right: none;
			    justify-content: center;
			}

			nav a {
			    text-align: center;
			}",
            );
        }
        html.push_str(
            "
		}

		.selected {
			font-style: italic;
        ",
        );
        html.push_str(&format!(
            "            border-top: 1px solid rgb({}, {}, {});",
            text_color.0, text_color.1, text_color.2
        ));
        html.push_str(&format!(
            "            border-bottom: 1px solid rgb({}, {}, {});",
            text_color.0, text_color.1, text_color.2
        ));
        html.push_str(
            "            
		}

		.info a {
			font-size: 1.5em;
		    padding: 0px;
		    margin: 0px;
		    text-decoration: none;
			cursor: pointer;
			font-style: italic;
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
  <body>
",
        );
        if titles.len() > 1 {
            html.push_str(
                "
    <nav>
    	<header>
        	<div class=\"info\"> <a href=\"https://github.com/dynalgo/dynalgo.github.io\">Dynalgo</a>
        	</div>
        </header>
        <ul id=\"menu\">",
            );

            for (i, tittle) in titles.iter().enumerate() {
                if i == title_idx {
                    html.push_str(
                        "
		<li class=\"selected\">",
                    );
                } else {
                    html.push_str(
                        "
		<li>",
                    );
                }
                html.push_str(&format!(
                    "<a href=\"{}.html\">{}</a>",
                    file_names[i], tittle
                ));
                html.push_str(
                    "
		</li>",
                );
            }

            html.push_str(
                "
        </ul>
    </nav>",
            );
        }

        html.push_str(
            "
    <section>",
        );
        for svg_content in contents {
            html.push_str(
                "
		<article>",
            );
            html.push_str(&svg_content);
            html.push_str(
                "
		</article>",
            );
        }
        html.push_str(
            "
    </section>
  </body>
</html>",
        );

        html
    }
}
