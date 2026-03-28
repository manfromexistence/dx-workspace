Forget about the model; default model changing as I myself manually will fix that. Now just work on the codex UI so that it can reflect some changes. As in codex UI we have to render our dx UI. Do the first step on that goal and create another UI stuff in the Codex UI.
Currently the Codex UI has all screen set to auto but the all screen is just showing the latest markdown and auto scrolling with no way to manually scroll the content. Study the Codex UI and on the right side of the Codex UI create a scroll bar so that we can scroll in the Codex UI messages. Currently you may find that Codex UI already has scroll but it has scroll on inline UI mode. As we are currently making the all screen true it is currently using the full screen but it is not using the scroll bar. It previously was only configured around having the native scroll bar but as on all screen there is no native scroll bar, that is why in the current Codex UI there is no way to scroll things. Please create a custom scroll bar for it. 

When we right click on the text auto pastes the text to the input but please on input box top right please create a border button called "pasted text" and in the input don't paste anything or submit anything so on rigth click the clipboard content may be pasted buy not auto sended

And also when we drag and drop any files or image, please show an input box in the top right button next to the pasted button. Make sure all the items are creating good UI in the top right button. Please make the button have a border and be small, only like the title input, so that the buttons are tight or small in width. Give a small button-like height so that they look like buttons correctly.

Now in our tui, the current tui is literally one of the best rust tui in the world so it also has a click handler. Make sure when we click on the buttons, like the top input box stop right buttons that we just created now, it will show a menu with our techone effect menu and show the title of the content, like the pasted text or image with the title or path details on the top left. On the main menu it will render the content. For the pasted text it will render the text inside of the pasted text button and for other image and other stuff, for now we will just render the menu and will render the image or content later; just text rendering is fine.

And in our input box if we paste something or input something with some sts, it's skipping even spaces, like it's including one space but excluding next space. Maybe it's for the with-space interaction that we have in add it in our input box for the voice input but it's kinda like backfiring. Make sure that it works correctly. Make sure that when we hold on Ctrl+Windows key (or any other same key on other desktops like macOS or Linux), it will still do the voice input that we are currently doing on the space key holding. Add another key trigger on the voice key and it will have the two key - ctrl+win/macos(command)/linux(system key)

And for our message list please generate the JSON file in our current workspace, like generate the JSON message list files in our current workspace or directly.
And currently if we are not focused on the chat input box, it still shows the same rainbow animation. Even when focusing on the chat input box, it still renders the same one. Please make sure that when we are not focusing on our text input box in our terminal TUI, it shows another animation reflecting that no user has focus on the text input.

And put the the paste and image or file button on the top right of the input box instead of the right inside center of the input box

and even through in the first there might be no focus but still the first time the cursor is rainbow and not having that not focus cursor - its activating that cursor later so please fix it

Now please do your search about the sound of a train and a train running and please implement the train running sound when we exit our screen Alongside our rainbow train ascii animation . Make sure that if we press the Ctrl+C command more times then please reset the train animation. That will make a satisfying exit for the user. 

And if there is more text then increase our text input box to max be five characters long. If it's bigger then please put a scroll bar in our input box too, like the scroll bar that we are having in the message list.

Now about our File drag and drop in our TUI. Please make sure that the file just shows the file name instead of the red sign or useless other stuff. Also even though I am adding one file, maybe it's structuring the file part differently so that's why it's counting two files so please fix it. Also make sure that when we drop any files it is kindly showing a dash border but please don't show any dash border or things like that. Even in the past it just shows the pasted text and shows this symbol "|" between the drop button and when we do ctrl+windows/mac/linux system button and hold onto it + press rigth click in input + ctrl+v Please show notifications that this button has been pressed.

Now instead of cancel text on the menu top right, please show three center dots with red, yellow, and green colors. When you click on the red circle dot, please close the menu.

Now currently in the message list when there is more space, we are always showing the scroll bar but please make sure we should only show the scroll bar when the user's mouse is on our Tui And also kindly our chat input box is only a static fixed-height chat input box now. Please make sure that if the text is more than that, our chat input box can grow up to a max of five characters long. When there's even more text, then please put a scroll bar, like the scroll bar that we have in our message list, in our chat input box so that our input box will be professional.

Now only on our message list scroll bar there is currently the scroll bar scrolling through lines. This is good. Now in the scroll bar please create a grid of position symbols, with a small height one-piece and a little bigger symbol, and put it at the top. The top symbol will reflect the fast chat input. If there are three chats currently in the message list, then at the top there will be one symbol, in the center there should be another one, and at the bottom there should be another one representing the three messages. When there is only one, then only the top symbol will be there. This is a system: as the message list grows the symbols will be added to represent all the chat book so that we can click on them and go to that specific message specifically. That is why we will not only have a line-by-line scroll system but also a chat message scroll system that will make the user experience much better.

On exit, the animation is getting the sound one time and one time it's not getting the sound, without literally changing anything. Maybe there's a bug so please fix it so that it can play the sound every time correctly. Now lower the volume of the train sounds.

now only alt+enter is detecting so when pressed shit/ctrl/alt+enter create new lines and put the diamon symbol on the scrollbar to have more padding or maring rigth and make those symbols clickable and when clicked show the field diamond and put the screen view to that place of that chat. As I told you divide the chat messages into the whole screen correctly. If there are two messages then on the top most top of the scroll bar there will be one diamond and on the bottom of the scroll bar there will be another one. This TUIpe of pattern will grow alongside the message list items are growing.

Now we have to enter the codex toy in our dx toy message list. Now as you can see we are not going to wrap the codex toy entirely on our dx toy. We are going to make a new brand new toy inside our dx toy, because that codex toy has so many UI problems and also more importantly it has no custom scrollbar. It only works on native terminal but in native terminal we can't show our animations because they require the full terminal support. Now here is the thing: we are not just going to create a sloppy Codex UI. We already have all the source code of the Codex UI so study the codebase and tell me: is there any way we can integrate the Codex UI markdown rendering and agent response rendering without taking the whole Codex UI with the input box and bad UI part? We are going to just use the learnings of the Codex UI, like how it implemented or called the Codex providers or the Codex AI response sandboxing and other stuff. We are just going to integrate it without the bad Codex UI itself.

then please show models of codex and also render models of codex and integrate it in our chage message list and make sure to keep our local model as the default infiniTUI unlimited one and in our provider's model implement Codex models 

And connect our input box bottom provider name with the actual provider so it will render the provider name and sync it with the provider menu and with the input box actions provided names. And make sure to connect this with the Corex real provider.

Rename the title from "Codex providers" to just "Providers".
Remove the custom sloppy UI you created.
Make the Providers list a manual list item.
Instead of the local infiniTUI, just put "infiniTUI" as the first model.
Use message list items instead of any weird sloppy UI because message link works correctly.
Make sure that when you click on the message list it successfully changes the model.

And make our whole TUI connect with Codex. Show the Codex path in our input action button and token info next to it. Completely integrate the Codex with our dx TUI.

Now you can look at the assets folder "F:\codex\codex-rs\dx\assets" And there you can find some new sounds that I have added. Now please implement all those sounds in our different screens like:
the matrix sound in the matrix screen
the wave sound in the wave screen
the firework sound in the firework screen
the rain sound in the rain screen
the space sound in the space-like screen
and things like that. Implement all the sounds in the screens correctly. If some screen doesn't match with the sound name then use curl and download the sound from the internet for that specific screen.

Now please integrate the whole codex into our dx TUI. Please learn from the codex TUI that is around the dx TUI code base and just implement it correctly in our dx TUI. When we run into the AI, make sure to show the answer from codex and fully integrate codex into our dx tui And please show me a list and tell me what we need to integrate in our dx tui of codex to implement the codex fully in our dx tui instead of the codex tui.

And in Codex code I previously added a local model running feature but in that local model running it was always just not finding the model. In dxTUI I implemented the local gduf model running fully.
Now you can do two things:
1. If the local model running of the Codex core integration I did is similar to dx local gduf model running, then just update it like the current dxTUI local gduf model running.
2. If not then remove the local model running entirely from the Codex code that I implemented earlier.
Currently if we try to add another provider, it is defaulting to local gduf running and the local gduf is not working too so it is creating a mess in Codex code. Implement it correctly or if it is hard then just remove it correctly because in our dxTUI I implemented it correctly. Do what is best.

And make it so that when there is no base URL it was defaulting to the local model. Please try to run the local model because I already implemented multi-provider in Codex TOI so make sure that we use that instead of that local model running and even worse, the provider character. 

And about your checklist bro, we literally diverted our plan to not run the Codex TUI inside our DX TUI because Codex TUI has a fundamental TUI problem. It's really good to just recreate it in our DX TUI as Codex TUI uses the Codex code and other protocol anyways so we will be doing the same thing instead of wrapping the correct UI inside our DX TUI.

Now we already set the Mistral API key in our operating system and we implemented the codex to work on the Mistral latest low model with the API key. In our dxTUI make sure Mistral is the default model with the codex and render the codex messages like the codex tui rendering in our dx message list correctly. 

Now in our DX folder we also have a binary about the real Codex TUI. Now please tell me what the real Codex TUI has with the Codex code that we don't have in our DX TUI. Make it check it out and tell me what we still need to do in our DX; then you might fully implement Codex. 
now we we have to do it in a way so that we will use the actual Codex tui code. Now we will only use the part that we needed for the connection. We'll ignore the part that the Codex tui specifically used for the tui itself.
Please create a checklist about how we can configure the current Codex tui so that we can implement them in our dx tui, in a way so that the unusable parts are not needed, like the totally tui-specific stuff. We don't need that; we only need the professional code that does the connection of Codex score with that tui so that we can implement it in our dx tui too. 

Now learn from the Codex TUI and in our dx TUI implement the similar learning from the Codex TUI. Implement this with the Codex dx TUI components like the menu and things like that. 
- External editor integration
- Approval popup UI
- Skills list UI
- Plugin marketplace UI

Now give me the brutal truth of the checklist of the features that Codex TUI has that our DX TUI doesn't have right now. Give me the brutal honest truth. 

Then what is the next step and tell me, around 100%, how much we are close to implementing the codex fully in our dx.

Please look at the root codex_tui_architecture.md file and look at the codex-rs/dx folder and tell me what is faster to implement. Is it faster to integrate dx, tui into codex? Is it faster to implement codex-tui into dx-tui?

Now in our codex-rs/dx folder codex-tui-dx binary please do this make sure we have full codex-tui inside the dx folder and don't change anything related to dx binary as there are two binary and only change the codex-tui-dx binary related files

So as you can see this is not an easy task so maybe ask for a better AI for help and make sure the better AI uses GitHub to fetch details. You have to mention that we are in a local file and that the Codex open-source GitHub repo has these files. Now users give me solutions to implement this feature. Make the help file like this. 

Now I have reverted the changes that you did because still the same problem. The thing is maybe the codex tUI is complex and the team behind it completely ignore the fact that we can pin the input box at the bottom. They were first of all working on making it an inline terminal UI, completely ignoring the fact that there can be a scenario when we have to put the input box at the bottom.
That is why all the files of codex are so big. We have to come up with a clever way. Currently in the top we are showing the box with codex model detail and a small slash command detail, then there comes a tip, and after that the input box and the status line. Now before the input box we can render a UI that gets the full height so that the input box will automatically be at the bottom. That is the way it gets to the bottom anyway. The input top part has got some more content and it moves the input box to the bottom and then the input box stays at the absolute bottom. We will use that in our favor so try to render a big big wall with a fixed height that is big enough to push the input box to the bottom. 

Now, we are in the in-game. In the src folder of codex-rs/dx folder, you can find the scrollbar.rs file. So, create another one and implement a custom scrollbar in the message list of the Codex tui. Make sure there are two binaries: the previous scrollbar is connected to another binary. So, we have to connect a new scrollbar with our current Codex-tui-dx binary.

If it's possible, please remove the mouse wheel interaction from the info box as it's conflicting with the message list.

So DX binary has an splash screen right? So please render that onboarding_screen in the codex tui as the default. Instead of the codex top card or tip details, and when there is a message, please show the message
