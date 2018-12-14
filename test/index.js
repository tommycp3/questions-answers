// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("app", "dist/bundle.json")


// activate the new instance
app.start()

test('a user can add a question', (t) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
  
  const result = app.call("qa", "main", "add_question", {
    question: {
      text: "How do I write a test?"
    }
  })

  // check for equality of the actual and expected results
  t.deepEqual(result, { address: " "})

  // ends this test
  t.end()
})
 