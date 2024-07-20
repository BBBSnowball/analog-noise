// This wouldn't have been possible without the infos here:
// https://github.com/Marus/cortex-debug/issues/104#issuecomment-1450108082

// This will not work unless the following bug is patched:
// https://github.com/Marus/cortex-debug/pull/961
// Edit /home/user/.vscode/extensions/marus25.cortex-debug-1.12.1/dist/extension.js,
// change "data" to "message" in `this.emit` call in `graphData` near `:1:144220`.

function MyAdvancedDecoder() {
}
MyAdvancedDecoder.prototype = {
    typeName: function () { return "abc"; },
    outputLabel: function() { return "def"; },
    synchronized: function() {},
    lostSynchronization: function() {},
    init: function (config, outputData, graphData) {
        this.count = 42;
        this.data = [];
        this.outputData = outputData;  // string => void
        this.graphData = graphData;  // number, string => void
    },
    softwareEvent: function (port, data) {
        this.count += data.byteLength;
        //var data2 = new Uint8Array(data.buffer);
        var data2 = new Int8Array(data.buffer, data.byteOffset, data.length);
        // This will print to a console under the OUTPUT tab ("SWO: def [type: abc]")
        //this.outputData(`${this.count},${data.length},${data2.byteLength}: ${data2}\n`);

        this.data.push.apply(this.data, data2)

        if (this.data.length >= 9) {
            this.outputData(`${this.count},${data.length},${this.data.length}: ${this.data}\n`);

            this.graphData(this.data[1], "T");
            this.graphData(this.data[4], "x");
            this.graphData(this.data[6], "y");
            this.graphData(this.data[8], "z");
            //this.graphData(10, "d");
            this.data.splice(0, 9);
        }
    },
};

//export default MyAdvancedDecoder;
exports.default = MyAdvancedDecoder;
