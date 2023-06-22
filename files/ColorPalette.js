//Displays and handles the color palette.
function ColorPalette() {
	//a list of web color strings
	this.colors = ["white", "silver", "gray", "black", "maroon", "red", "purple",
		"orange", "pink", "fuchsia", "green", "lime", "olive", "yellow", "navy",
		"blue", "teal", "aqua"
	];
    this.selectedColor = "white";
	//make the start color be white

	var self = this;

	var colorClick = function() {
		//remove the old border
		var current = select("#" + self.selectedColor + "Swatch");
		current.style("border", "0");

		//get the new color from the id of the clicked element
		var c = this.id().split("Swatch")[0];

		//set the selected color and fill and stroke
		self.selectedColor = c;
		fill(c);
		stroke(c);

		//add a new border to the selected color
		this.style("border", "2px solid blue");
	}

	//load in the colors
	this.loadcolors = function() {
		//set the fill and stroke properties to be black at the start of the programme
		//running
		fill(this.colors[0]);
		stroke(this.colors[0]);

		//for each color create a new div in the html for the colorSwatches
		for (var i = 0; i < this.colors.length; i++) {
			var colorID = this.colors[i] + "Swatch";

			//using p5.dom add the swatch to the palette and set its background color
			//to be the color value.
			var colorSwatch = createDiv()
			colorSwatch.class('colorSwatches');
			colorSwatch.id(colorID);

			select(".colorPalette").child(colorSwatch);
			select("#" + colorID).style("background-color", this.colors[i]);
			colorSwatch.mouseClicked(colorClick)
		}

		select(".colorSwatches").style("border", "2px solid blue");
	};
	//call the loadcolors function now it is declared
	this.loadcolors();
}
