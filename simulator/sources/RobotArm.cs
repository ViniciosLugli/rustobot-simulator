using Godot;
using System;



public struct Border
{
	public float min;
	public float max;

	public Border(float min, float max)
	{
		this.min = min;
		this.max = max;
	}

	public float Parse(float value)
	{
		switch (value)
		{
			// Check if the value is in the borders and set it if it is
			case float v when (v < this.min):
				GD.Print(String.Format("Value {0} is less than the minimum border {1}, setting it to {1}", v, this.min));
				value = this.min;
				break;
			// Same as above but with the maximum border
			case float v when (v > this.max):
				GD.Print(String.Format("Value {0} is greater than the maximum border {1}, setting it to {1}", v, this.max));
				value = this.max;
				break;
			// If the value is in the borders, set it
			default:
				break;
		}

		return value;
	}
}

public struct Position
{
	// Positions
	public float x;
	public float y;
	public float z;

	public Position(float x = 0, float y = 0, float z = 0)
	{
		this.x = x;
		this.y = y;
		this.z = z;
	}

	public override string ToString()
	{
		return String.Format("({0}, {1}, {2})", x, y, z); // Better for debugging
	}


	/*
		Operators overloading for Position struct
	*/

	public static Position operator +(Position a, Position b)
	{
		return new Position(a.x + b.x, a.y + b.y, a.z + b.z);
	}

	public static Position operator -(Position a, Position b)
	{
		return new Position(a.x - b.x, a.y - b.y, a.z - b.z);
	}

	public static Position operator *(Position a, float b)
	{
		return new Position(a.x * b, a.y * b, a.z * b);
	}

	public static Position operator /(Position a, float b)
	{
		return new Position(a.x / b, a.y / b, a.z / b);
	}

	public static bool operator ==(Position a, Position b)
	{
		return a.x == b.x && a.y == b.y && a.z == b.z;
	}

	public static bool operator !=(Position a, Position b)
	{
		return a.x != b.x || a.y != b.y || a.z != b.z;
	}

	public override bool Equals(object obj)
	{
		if (obj == null || GetType() != obj.GetType()) // Check for null and compare run-time types.
		{
			return false; // Not the same type
		}

		Position p = (Position)obj; // Cast to Position struct
		return (x == p.x) && (y == p.y) && (z == p.z); // Check for equality
	}

	public override int GetHashCode()
	{
		// Generate hash code for Position struct using XOR
		// Necessary to hashcode comparison
		return x.GetHashCode() ^ y.GetHashCode() ^ z.GetHashCode();
	}
}

public class WorldPosition
{
	// Positions
	private Position _position;

	public float x
	{
		get { return this.position.x; }
		set
		{
			this._position.x = this.b_x.Parse(value); // Parse the position to check if it is in the borders
		}
	}
	public float y
	{
		get { return this.position.y; }
		set
		{
			this._position.y = this.b_y.Parse(value); // Parse the position to check if it is in the borders
		}
	}

	public float z
	{
		get { return this.position.z; }
		set
		{
			this._position.z = this.b_z.Parse(value); // Parse the position to check if it is in the borders
		}
	}

	public Position position
	{
		get { return this._position; }
		set
		{
			this.x = value.x;
			this.y = value.y;
			this.z = value.z;
		}
	}

	// Borders
	public Border b_x;
	public Border b_y;
	public Border b_z;

	public WorldPosition(Position position = new Position())
	{
		this.position = position;
		this.b_x = new Border(-135, 135); // 270 is the maximum angle X of the robot arm
		this.b_y = new Border(-60, 60); // 120 is the maximum angle Y of the robot arm
		this.b_z = new Border(-30, 30); // 120 is the maximum angle Z of the robot arm
	}
}

public struct Velocity
{
	private float _value;
	public float value
	{
		get { return this._value; }
		set
		{
			this._value = this.border.Parse(value); // Parse the velocity to check if it is in the borders
		}
	}
	public Border border;

	public Velocity(float value)
	{
		this.border = new Border(0, 100);
		this._value = this.border.Parse(value);
	}

	public float ToDuration(float distance)
	{
		// Calculate the duration of the movement based on the velocity and the distance:
		return Math.Abs(distance / this.value);
	}
}

public struct HeadRotation
{
	private float _value;
	public float value
	{
		get { return this._value; }
		set
		{
			this._value = this.border.Parse(value); // Parse the velocity to check if it is in the borders
		}
	}
	public Border border;

	public HeadRotation()
	{
		this.border = new Border(-90, 90);
		this._value = 0;
	}
}

public partial class RobotArm : Node3D
{

	[Signal]
	public delegate void EndedQueueEventHandler();

	private WorldPosition world;
	private HeadRotation head_rotation;
	private Velocity velocity;

	private Tween gtween;
	private bool is_queued = false;

	private Node3D Arm1to2
	{
		get { return (Node3D)GetNode("Arm1to2"); } // Joint 1
	}

	private Node3D Arm2to3
	{
		get
		{
			return (Node3D)GetNode("Arm1to2/Arm2to3"); // Joint 2
		}
	}

	private Node3D Arm3to4
	{
		get
		{
			return (Node3D)GetNode("Arm1to2/Arm2to3/Arm3to4"); // Join 2 - For adjust only
		}
	}

	private MeshInstance3D Claw
	{
		get
		{
			return (MeshInstance3D)GetNode("Arm1to2/Arm2to3/Arm3to4/MeshArm3to4/Claw"); // Claw rotation mesh
		}
	}


	private void UpdateXPosition()
	{
		/* Update X position */
		// distance = world.x - this.Arm1to2.RotationDegrees.X
		float duration_x_move = this.velocity.ToDuration(this.world.x - this.Arm1to2.RotationDegrees.X);
		// X of arm is Y of world of Arm1to2
		this.gtween.TweenProperty(this.Arm1to2, "rotation_degrees:y", this.world.x, duration_x_move);
	}

	private void UpdateYPosition()
	{
		/* Update Y position */
		// distance = world.y - this.Arm1to2.RotationDegrees.Y
		float duration_y_move = this.velocity.ToDuration(this.world.y - this.Arm1to2.RotationDegrees.Y);
		// Y of arm is X of world of Arm1to2
		this.gtween.TweenProperty(this.Arm1to2, "rotation_degrees:x", this.world.y, duration_y_move);
	}

	private void UpdateZPosition()
	{
		/* Update Z position */
		// distance = world.z - this.Arm2to3.RotationDegrees.X
		float duration_z_move = this.velocity.ToDuration(this.world.z - this.Arm2to3.RotationDegrees.X);
		// Z of arm is X of world of Arm2to3
		this.gtween.TweenProperty(this.Arm2to3, "rotation_degrees:x", this.world.z, duration_z_move);
	}

	private void UpdateRPosition()
	{
		/* Update R position */
		// distance = head_rotation.value - this.Claw.RotationDegrees.Z
		float duration_r_move = this.velocity.ToDuration(this.head_rotation.value - this.Claw.RotationDegrees.Z);
		// R of arm is Z of world of Claw
		this.gtween.TweenProperty(this.Claw, "rotation_degrees:z", this.head_rotation.value, duration_r_move);
	}


	private void UpdatePivots()
	{
		this.Arm3to4.RotationDegrees = new Vector3(-this.Arm1to2.RotationDegrees.X, this.Arm3to4.RotationDegrees.Y, this.Arm3to4.RotationDegrees.Z); // Update pivot of Arm3to4 to Arm1to2 X
	}

	private void SetupTween()
	{
		this.gtween = this.GetTree().CreateTween();
		this.gtween.SetParallel(!is_queued);
		this.gtween.Connect("finished", Callable.From(() =>
		{
			this.EmitSignal(SignalName.EndedQueue);
			this.gtween.Kill();
		}));
	}

	private void RuntimeUpdate()
	{
		this.SetupTween();
		this.UpdateXPosition();
		this.UpdateYPosition();
		this.UpdateZPosition();
		this.UpdateRPosition();
	}

	public void Move(Position position, bool relative = true)
	{
		// Move the robot arm to the specified position relative to the current position
		this.world.position = relative ? this.world.position + position : position;
	}

	public void MoveX(float value, bool relative = true)
	{
		this.world.x = relative ? this.world.x + value : value;
	}

	public void MoveY(float value, bool relative = true)
	{
		this.world.y = relative ? this.world.y + value : value;
	}

	public void MoveZ(float value, bool relative = true)
	{
		this.world.z = relative ? this.world.z + value : value;
	}

	public void Rotate(float rotation, bool relative = true)
	{
		this.head_rotation.value = relative ? this.head_rotation.value + rotation : rotation;
	}

	public void SendRequest()
	{
		this.RuntimeUpdate();
	}

	public void SetQueue(bool is_queued)
	{
		this.is_queued = is_queued;
	}

	public bool IsBusy()
	{
		return this.gtween.IsRunning();
	}

	public void KillProcesses()
	{
		this.gtween.Kill();
	}

	public override void _Ready()
	{
		this.velocity = new Velocity(50);
		this.world = new WorldPosition();
		this.head_rotation = new HeadRotation();

		this.SetQueue(false);
	}

	public override void _Process(double delta)
	{
		this.UpdatePivots();
		this.GetNode<Label>("../Control/Label").Text = $"Current point:\nX: {Math.Round(this.Arm1to2.RotationDegrees.X, 2)} | Y: {Math.Round(this.Arm1to2.RotationDegrees.Y, 2)} | Z: {Math.Round(this.Arm2to3.RotationDegrees.X, 2)} | R: {Math.Round(this.Claw.RotationDegrees.Z, 2)}";
	}
}
