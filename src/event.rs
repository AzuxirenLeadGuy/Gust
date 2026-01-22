//! Module to handle keyboard and mouse event one day
use crate::window::Window;
use glfw::{self, GlfwReceiver};

#[macro_export]
/// Should expand from pressed!(W) -> Events::Key(Key::W, _, Action::Pressed, _)
macro_rules! pressed {
    ($x:ident) => {
        glfw::WindowEvent::Key(Key::$x, _, Action::Press, _)
    };
}

#[macro_export]
macro_rules! release {
    ($x:ident) => {
        glfw::WindowEvent::Key(Key::$x, _, Action::Release, _)
    };
}

#[macro_export]
macro_rules! repeat {
    ($x:ident) => {
        glfw::WindowEvent::Key(Key::$x, _, Action::Repeat, _)
    };
}

pub type EventMessage<'a> = glfw::FlushedMessages<'a, (f64, glfw::WindowEvent)>;

/// Event Wrap glfwEvent data
pub type Event = (f64, glfw::WindowEvent);
pub type EventReceiver = GlfwReceiver<(f64, glfw::WindowEvent)>;

pub struct EventHandler<'a> {
    receiver: &'a EventReceiver,
}

impl<'a> EventHandler<'a> {
    pub fn new(window: &Window) -> EventHandler<'_> {
        EventHandler {
            receiver: window.event(),
        }
    }

    pub fn fetch(&self) -> EventIterator<'_> {
        EventIterator::from(&*self)
    }
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.fmsg.next()
    }
}

/// EventIterator is an iterator on eventMessage to glob glfw Event system
pub struct EventIterator<'a> {
    fmsg: EventMessage<'a>,
}

impl<'a, 'b> From<&'a EventHandler<'b>> for EventIterator<'a> {
    fn from(var: &'a EventHandler) -> EventIterator<'a> {
        EventIterator {
            fmsg: glfw::flush_messages(var.receiver),
        }
    }
}

pub enum EventType {
    Key,
    Pos,
    Close,
    Size,
    Refresh,
    Focus,
    Char,
    CharMods,
    MouseButton,
    CursorPos,
    CursorEnter,
    Scroll,
    FrameBuffer,
}
